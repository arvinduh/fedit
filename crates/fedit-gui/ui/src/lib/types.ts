export interface ItemDto {
  name: string;
  kind: string;
  uses: number;
  maxUses: number;
  isEquipped: boolean;
  forgeMt?: number;
  forgeHit?: number;
  forgeCrt?: number;
}

export interface SkillDto {
  name: string;
  glyph: string;
  description: string;
}

export interface UnitDto {
  id: number;
  slug: string;
  name: string;
  title: string;
  className: string;
  level: number;
  exp: number;
  curHp: number;
  maxHp: number;
  boots: number;
  stats: [number, number, number, number, number, number, number, number];
  caps: [number, number, number, number, number, number, number, number];
  bases: [number, number, number, number, number, number, number, number];
  additions: [number, number, number, number, number, number, number, number];
  growths: [number, number, number, number, number, number, number, number];
  inventory: ItemDto[];
  skills: SkillDto[];
}

export interface SaveSlotDto {
  slotIndex: number;
  chapter: string;
  difficulty: string;
  mode: string;
  playTime: string;
  gold: number;
  units: UnitDto[];
}
