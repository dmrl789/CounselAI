from __future__ import annotations

from datetime import datetime, timezone
from typing import List, Literal, Optional

from pydantic import BaseModel, Field, field_validator


class Party(BaseModel):
    name: str
    role: Literal[
        "Ricorrente", "Resistente", "Attore", "Convenuto", "Cliente", "Controparte"
    ]

    @field_validator("name")
    @classmethod
    def name_must_not_be_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Name cannot be empty")
        return v.strip()


class CaseFile(BaseModel):
    case_id: str
    client: Party
    parties: List[Party] = Field(default_factory=list)
    facts: List[str] = Field(default_factory=list)
    jurisdiction: Optional[str] = None
    applicable_law: List[str] = Field(default_factory=list)
    created_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))

    @field_validator("case_id")
    @classmethod
    def case_id_must_not_be_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Case ID cannot be empty")
        return v.strip()


class ReasoningNode(BaseModel):
    id: str
    claim: str
    supports: List[str] = Field(default_factory=list)
    citations: List[str] = Field(default_factory=list)
    timestamp: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))


class ReasoningTree(BaseModel):
    root_id: str
    nodes: List[ReasoningNode]
    summary: str


class Opinion(BaseModel):
    case_id: str
    title: str
    summary: str
    recommendations: List[str]
    citations: List[str]
    generated_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))
