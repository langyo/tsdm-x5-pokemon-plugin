export const typeColors: Record<string, string> = {
  '火': '#F08030',
  '水': '#6890F0',
  '草': '#78C850',
  '电': '#F8D030',
  '冰': '#98D8D8',
  '斗': '#C03028',
  '毒': '#A040A0',
  '地': '#E0C068',
  '飞': '#A890F0',
  '超': '#F85888',
  '虫': '#A8B820',
  '岩': '#B8A038',
  '鬼': '#705898',
  '龙': '#7038F8',
  '恶': '#705848',
  '钢': '#B8B8D0',
  '妖': '#EE99AC',
  '普': '#A8A878',
}

export function getTypeColor(type: string): string {
  return typeColors[type] || '#A8A878'
}

export function hpClass(hp: number, maxHp: number): string {
  if (maxHp <= 0) return 'hp-red'
  const pct = hp / maxHp
  if (pct > 0.5) return 'hp-green'
  if (pct > 0.2) return 'hp-yellow'
  return 'hp-red'
}

export function genderLabel(sex: number): string {
  if (sex === 1) return '♂'
  if (sex === 2) return '♀'
  return ''
}

export const IMG_PATH = 'https://img.tsdm39.com/Pokemon'

export function spriteUrl(pmno: number): string {
  return `${IMG_PATH}/spm/${pmno}.gif`
}

export function battleSpriteUrl(pmno: number): string {
  return `${IMG_PATH}/pm/${pmno}.gif`
}
