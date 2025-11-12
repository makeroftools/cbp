from pydantic_settings import BaseSettings
from pydantic import Field
from typing import Literal

class AccountingConfig(BaseSettings):
    model_config = {
        'env_file': '.env',
        'env_file_encoding': 'utf-8',
    }
    precision: int = Field(default=4, description="Decimal precision")
    rounding: Literal['ROUND_HALF_UP', 'ROUND_HALF_EVEN', 'ROUND_DOWN', 'ROUND_UP'] = Field(default='ROUND_HALF_UP')
    validate_on_add: bool = Field(default=True)
    log_level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR'] = Field(default='INFO')
    persistence_format: Literal['parquet', 'feather'] = Field(default='parquet')
    log_transactions: bool = Field(default=False)