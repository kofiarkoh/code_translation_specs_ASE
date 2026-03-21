#include <stdio.h>

int main(void) {
    int value;
    int case_counter = 0;

    for (;;) {
        /* Specification: no explicit handling for end-of-input or read failures.
           We therefore ignore scanf's return value and proceed with whatever
           value ends up in 'value' (which may be indeterminate). */
        (void)scanf("%d", &value);

        case_counter += 1;

        if (value == 0) {
            break;
        }

        printf("Case %d: %d\n", case_counter, value);
    }

    return 0;
}
