-- EuclideanDivisionLock.lean
-- Lean 4 + Mathlib4 — Euclidean Division Lock

import Mathlib.Data.Int.Basic
import Mathlib.Tactic

theorem euclidean_division_lock (a b : Int) (hb : b ≠ 0) :
    ∃! (q r : Int), a = b * q + r ∧ 0 ≤ r ∧ r < Int.natAbs b := by
  have h_main : ∃! (q r : Int), a = b * q + r ∧ 0 ≤ r ∧ r < Int.natAbs b := by
    have h1 : ∃ (q r : Int), a = b * q + r ∧ 0 ≤ r ∧ r < Int.natAbs b := by
      use a / b, a % b
      constructor
      · rw [Int.ediv_add_emod]
      constructor
      · apply Int.emod_nonneg
        norm_cast; omega
      · apply Int.emod_lt_of_pos
        cases' abs_cases b with h6 h6 <;> norm_cast at h6 ⊢ <;> omega
    obtain ⟨q, r, hq, hr1, hr2⟩ := h1
    refine' ⟨q, r, ⟨hq, hr1, hr2⟩, _⟩
    rintro ⟨q', r', hq', hr'1, hr'2⟩
    have h5 : b * q + r = b * q' + r' := by linarith
    have h6 : r' - r = b * (q - q') := by linarith
    have h11 : q - q' = 0 := by
      have h14 : b ≠ 0 := hb
      by_contra h
      have h16 : (q - q' : Int) ≠ 0 := h
      have h17 : (q - q' : Int) ≥ 1 ∨ (q - q' : Int) ≤ -1 := by
        by_cases h18 : (q - q' : Int) ≥ 1
        · exact Or.inl h18
        · have : (q - q' : Int) ≤ -1 := by omega
          exact Or.inr this
      cases h17 with
      | inl h17 => cases' abs_cases b with h20 h20 <;> nlinarith
      | inr h17 => cases' abs_cases b with h20 h20 <;> nlinarith
    have h12 : q = q' := by linarith
    have h13 : r = r' := by linarith
    exact ⟨by linarith, by linarith⟩
  exact h_main

def java_mod_adjust (a b : Int) (hb : b ≠ 0) : Int :=
  let r := a % b
  if r < 0 then r + Int.natAbs b else r

theorem java_adjust_yields_euclidean (a b : Int) (hb : b ≠ 0) :
    let r_euclid := java_mod_adjust a b hb in
    0 ≤ r_euclid ∧ r_euclid < Int.natAbs b ∧
    ∃ (q : Int), a = b * q + r_euclid := by
  dsimp only [java_mod_adjust]
  split_ifs
  · sorry
  · constructor
    · apply Int.emod_nonneg; norm_cast; omega
    constructor
    · apply Int.emod_lt_of_pos; cases' abs_cases b with h4 h4 <;> norm_cast at h4 ⊢ <;> omega
    · use a / b; rw [Int.ediv_add_emod]

lemma abs_cases (b : Int) : b ≥ 0 ∨ b < 0 := by
  by_cases h : 0 ≤ b
  · exact Or.inl h
  · exact Or.inr (by omega)
