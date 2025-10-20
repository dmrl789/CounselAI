from __future__ import annotations

from datetime import datetime
from typing import List, Literal, Optional

from pydantic import BaseModel, Field


class Party(BaseModel):
    name: str
    role: Literal[
        "Ricorrente", "Resistente", "Attore", "Convenuto", "Cliente", "Controparte"
    ]


class CaseFile(BaseModel):
    case_id: str
    client: Party
    parties: List[Party] = Field(default_factory=list)
    facts: List[str] = Field(default_factory=list)
    jurisdiction: Optional[str] = None
    applicable_law: List[str] = Field(default_factory=list)
    created_at: datetime = Field(default_factory=datetime.utcnow)


class ReasoningNode(BaseModel):
    id: str
    claim: str
    supports: List[str] = Field(default_factory=list)
    citations: List[str] = Field(default_factory=list)
    timestamp: datetime = Field(default_factory=datetime.utcnow)


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
    generated_at: datetime = Field(default_factory=datetime.utcnow)
