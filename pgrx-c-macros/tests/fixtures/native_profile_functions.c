/* Original C functions supply the direct and indirect char ABI; Rust never defines a callback. */
/* Transport every char bit pattern through the original compiler's parameter and return ABI. */
char profile_char_identity(char value) {
    return value;
}
/* Keep the callback's code pointer and prototype owned by C, including char extension rules. */
ProfileCharCallback profile_char_callback(void) {
    return profile_char_identity;
}
