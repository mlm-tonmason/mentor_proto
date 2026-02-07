# Project Knowledge Base

## Specification
The detailed business requirements and domain logic definitions are located in the `specs/` directory.
Always refer to these files for the source of truth regarding project logic.

- [Specs Directory](./specs/)
- [Domain Models](./specs/domain_models.md)
- [Client Service Specs](./specs/client_service.md)

## Proto Structure
The protobuf definitions follow the structure:
- `mentor/types/`: Common domain entities (User, Bot, Chat).
- `mentor/client/`: Client-facing API services (Account, Market, Chat).
- `mentor/admin/`: Admin-facing API services.

## Agent Rules
See `.agent/rules/` for detailed guidelines.
