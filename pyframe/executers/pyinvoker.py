import inspect
from typing import (
    Any,
    Awaitable,
    Callable,
    Dict,
    Optional,
    ParamSpec,
    Tuple,
    Type,
    TypeVar,
    Union,
    overload,
)

from pydantic import BaseModel, ValidationError

from pyframe.executers.executer import ConnectionsProtocol, ProtocolHandlerBase

P = ParamSpec("P")
R = TypeVar("R")

# Generisch typisierte Command-Handler-Definition
CommandFunc = Callable[P, Union[R, Awaitable[R]]]


class PyInvoker(ProtocolHandlerBase):
    def __init__(self) -> None:
        # Speichert die Kommandos
        self.commands: Dict[str, CommandFunc[..., Any]] = {}
        # Speichert für jedes Kommando alle Parameter (falls Pydantic-Modell)
        self.models: Dict[str, Dict[str, Union[Type[BaseModel], Type[Any]]]] = {}
        ConnectionsProtocol.add_protocol("pyinvoker", self)

    def register(
        self, name: str, *args: P.args, **kwargs: P.kwargs
    ) -> Callable[[CommandFunc[P, R]], CommandFunc[P, R]]:
        """
        Dekorator, um einen Command-Handler zu registrieren.

        :param name: Name des Kommandos.
        :param args: Zusätzliche optionale Argumente für die Registrierung.
        :param kwargs: Zusätzliche optionale Keyword-Argumente.
        :return: Den dekorierten Handler.
        """

        def wrapper(func: CommandFunc[P, R]) -> CommandFunc[P, R]:
            sig: inspect.Signature = inspect.signature(func)
            param_models: Dict[str, Union[Type[BaseModel], Type[Any]]] = {}

            # ✅ Jetzt für ALLE Parameter prüfen:
            for param_name, param in sig.parameters.items():
                annotation: Any = param.annotation
                if annotation is inspect._empty:
                    param_models[param_name] = dict
                else:
                    param_models[param_name] = annotation

            self.models[name] = param_models
            self.commands[name] = func
            return func

        return wrapper

    async def trigger(self, data: Optional[Dict[str, Any]]) -> Any:
        if data is None:
            raise ValueError("Data payload is missing")

        cmd = data.get("cmd")
        if cmd is None:
            raise ValueError("Command is missing in data payload")

        # alle restlichen Parameter
        kwargs = {k: v for k, v in data.items() if k != "cmd"}
        result = await self.trigger_command(cmd, **kwargs)
        return result

    async def trigger_command(self, cmd: str, **kwargs: Any) -> Any:
        if cmd not in self.commands:
            raise KeyError(f"Command not registered: {cmd}")

        handler: CommandFunc[..., Any] = self.commands[cmd]
        param_models: Dict[str, Union[Type[BaseModel], Type[Any]]] = self.models[cmd]

        validated_kwargs = {}
        for param_name, param_value in kwargs.items():
            model_cls = param_models.get(param_name, None)
            if isinstance(model_cls, type) and issubclass(model_cls, BaseModel):
                if isinstance(param_value, dict):
                    validated_kwargs[param_name] = model_cls(**param_value)
                else:
                    validated_kwargs[param_name] = model_cls(
                        **{param_name: param_value}
                    )
            else:
                validated_kwargs[param_name] = param_value

        if inspect.iscoroutinefunction(handler):
            result: Any = await handler(**validated_kwargs)
        else:
            result = handler(**validated_kwargs)

        if isinstance(result, BaseModel):
            return result.model_dump_json(by_alias=True, indent=2)
        return result

    def available_commands(
        self,
    ) -> Dict[str, Dict[str, Union[Type[BaseModel], Type[Any]]]]:
        """
        Gibt alle verfügbaren Kommandos und deren Parameter-Typen zurück.
        """
        return self.models.copy()


# Angenommen: registry ist bereits definiert
registry = PyInvoker()


# Typüberladung für mehr Klarheit – mit und ohne Parameter nutzbar
@overload
def command(func: CommandFunc[P, R]) -> CommandFunc[P, R]: ...


@overload
def command(
    *, name: Optional[str] = None
) -> Callable[[CommandFunc[P, R]], CommandFunc[P, R]]: ...


def command(
    func: Optional[CommandFunc[P, R]] = None, *, name: Optional[str] = None
) -> Union[Callable[[CommandFunc[P, R]], CommandFunc[P, R]], CommandFunc[P, R]]:
    """
    Decorator zum Registrieren von Command-Funktionen.

    :param func: Direkt dekorierte Funktion (bei @command).
    :param name: Optionaler Befehlsname.
    :return: Den dekorierten Handler.
    """

    def decorator(fn: CommandFunc[P, R]) -> CommandFunc[P, R]:
        cmd_name = name or fn.__name__
        return registry.register(cmd_name)(fn)

    if func is None:
        return decorator
    return decorator(func)
