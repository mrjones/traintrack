import { DimensionState, VisibilityState } from '../src/filter-control'
import { webclient_api } from '../src//webclient_api_pb';

describe("VisibilityState", () => {
  it("has a default, empty state which makes sense", () => {
    let vstate = VisibilityState.parseFromSpec("");

    expect(vstate.isCombined()).toBeFalsy();

    // Any line or direction is enabled by default
    expect(vstate.includesLine("N")).toBeTruthy();
    expect(vstate.includesDirection(webclient_api.Direction.UPTOWN)).toBeTruthy();
  });

  it("is toggleable", () => {
    let vstate = VisibilityState.parseFromSpec("");

    expect(vstate.isCombined()).toBeFalsy();
    vstate.toggleCombined();
    expect(vstate.isCombined()).toBeTruthy();

    expect(vstate.includesLine("N")).toBeTruthy();
    vstate.toggleLine("N");
    expect(vstate.includesLine("N")).toBeFalsy();

    expect(vstate.includesDirection(webclient_api.Direction.UPTOWN)).toBeTruthy();
    vstate.toggleDirection(webclient_api.Direction.UPTOWN);
    expect(vstate.includesDirection(webclient_api.Direction.UPTOWN)).toBeFalsy();
  });

  it("parses serialized state", () => {
    let original = VisibilityState.parseFromSpec("");

    original.toggleCombined();
    original.toggleLine("N");
    original.toggleDirection(webclient_api.Direction.UPTOWN);

    expect(original.isCombined()).toBeTruthy();
    expect(original.includesLine("N")).toBeFalsy();
    expect(original.includesDirection(webclient_api.Direction.UPTOWN)).toBeFalsy();

    let restored = VisibilityState.parseFromSpec(original.toSpec());

    expect(restored.isCombined()).toBe(original.isCombined());
    expect(restored.includesLine("N")).toBe(original.includesLine("N"));
    expect(restored.includesDirection(webclient_api.Direction.UPTOWN)).toBe(original.includesDirection(webclient_api.Direction.UPTOWN));
  });

  it("handles express trains", () => {
    let original = VisibilityState.parseFromSpec("");

    original.toggleLine("7");
    original.toggleLine("7X");
    expect(original.includesLine("7")).toBeFalsy();
    expect(original.includesLine("7X")).toBeFalsy();

    let restored = VisibilityState.parseFromSpec(original.toSpec());
    expect(restored.includesLine("7")).toBeFalsy();
    expect(restored.includesLine("7X")).toBeFalsy();
  });


  it("Supports legacy specs", () => {
    // Originally a spec had one character per element: "-:-ACE::"
    // However, that could not support lines which had two characters,
    // for example, "7X", "FX" or "GS".
    //
    // So the spec was updated to use an explicit delimiter, '.', which
    // changed the format to look like "-:-A.C.E::".
    //
    // Since the spec is in the URL people might have bookmarked the old
    // spec, and we want to make sure it still works:

    let v1 = VisibilityState.parseFromSpec("-:-ACE::");

    expect(v1.includesLine("A")).toBeFalsy();
    expect(v1.includesLine("C")).toBeFalsy();
    expect(v1.includesLine("E")).toBeFalsy();

    expect(v1.includesLine("L")).toBeTruthy();
  });

  it("doesn't mistake single multi-letter line for a legacy spec", () => {
    let original = VisibilityState.parseFromSpec("");

    original.toggleLine("7X");
    expect(original.includesLine("7X")).toBeFalsy();

    let restored = VisibilityState.parseFromSpec(original.toSpec());

    expect(restored.includesLine("7X")).toBeFalsy();

  });

});
