from typing import Any
import sillyorm
from .. import registry


class MonetaryField(sillyorm.fields.Float):
    def __init__(self, digits: int = 2) -> None:
        self.digits = digits
        super().__init__()

    def _convert_type_set(self, value: Any) -> Any:
        if value is None:
            value = 0.0
        val = super()._convert_type_set(value)
        return round(val, self.digits)

    def _convert_type_get(self, value: Any) -> Any:
        val = super()._convert_type_get(value)
        if value is None:
            value = 0.0
        return round(val, self.digits)


@registry.register_model
class AccountAccount(sillyorm.model.Model):
    _name = "account_account"

    name = sillyorm.fields.String()


@registry.register_model
class AccountEntry(sillyorm.model.Model):
    _name = "account_entry"

    lines = sillyorm.fields.One2many("account_entry_line", "entry")

    # When this field is set to true the entry may not be changed! This field may also not be manually set to true!
    validated = sillyorm.fields.Boolean()

    def validate(self):
        credit = 0.0
        debit = 0.0
        for line in self.lines:
            credit += line.credit
            debit += line.debit
        credit = round(credit, 2)
        debit = round(debit, 2)
        if credit == debit:
            self.validated = True
        else:
            self.validated = False
            raise Exception("unable to validate. Credit is not equal to debit")


@registry.register_model
class AccountEntryLine(sillyorm.model.Model):
    _name = "account_entry_line"

    entry = sillyorm.fields.Many2one("account_entry")
    account = sillyorm.fields.Many2one("account_account")
    credit = MonetaryField()
    debit = MonetaryField()
