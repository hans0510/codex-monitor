# Quick Task 260708-jga: Reduce Lulu pet minimum size by half

## Scope

Lower the minimum allowed Lulu desktop pet size from the current 190px width to 95px width while leaving the default size and maximum size unchanged.

## Tasks

1. Update the size slider lower bound in the desktop UI.
   - Verify: the range input minimum is 95.
2. Update runtime size clamping to match the new lower bound.
   - Verify: saved or entered values below 95 clamp to 95, while max/default behavior stays unchanged.
3. Run focused validation.
   - Verify: text search confirms old 190px lower bound is gone from the size control logic, and project tests still pass.
