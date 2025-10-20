from __future__ import annotations
from typing import List, Optional, Literal
from pydantic import BaseModel, Field, field_validator
from datetime import datetime, timezone


class Party(BaseModel):
    name: str = Field(min_length=1, description="Party name cannot be empty")
    role: Literal["Ricorrente", "Resistente", "Attore", "Convenuto", "Cliente", "Controparte"]
    
    @field_validator('name')
    @classmethod
    def validate_name(cls, v):
        if not v or not v.strip():
            raise ValueError("Party name cannot be empty")
        return v.strip()


class CaseFile(BaseModel):
    case_id: str = Field(min_length=1, description="Case ID cannot be empty")
    client: Party
    parties: List[Party] = Field(default_factory=list)
    facts: List[str] = Field(default_factory=list)
    jurisdiction: Optional[str] = None
    applicable_law: List[str] = Field(default_factory=list)
    created_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))
    
    @field_validator('case_id')
    @classmethod
    def validate_case_id(cls, v):
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
