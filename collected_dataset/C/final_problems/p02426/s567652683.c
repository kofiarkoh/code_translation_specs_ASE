/*
 * ID: ITP2_10_C
 * Problem: Bit Flag
 */

#include <stdio.h>

int main(void)
{
    unsigned long a = 0;
    unsigned long mask[10] = {0};
    const unsigned long fullbit = -1; // 0xffffffffffffffff
    int i, n_mask, n_query;

    scanf("%d", &n_mask);

    for (i = 0; i < n_mask; i++) {
        int j, n_bit, bit;
        scanf("%d", &n_bit);
        for (j = 0; j < n_bit; j++) {
            scanf("%d", &bit);
            mask[i] |= (1L << bit);
        }
    }

    scanf("%d", &n_query);
    for (i = 0; i < n_query; i++) {
        int query, imask;
        int j, count;
        scanf("%d %d", &query, &imask);
        switch (query) {
            case 0: // test(i)
                printf("%d\n", (a & (0x01L << imask)) == 0 ? 0 : 1);
                break;
            case 1: // set(i);
                a |= mask[imask];
                break;
            case 2: // clear(i);
                a &= fullbit ^ mask[imask];
                break;
            case 3: // flip(i);
                a ^= mask[imask];
                break;
            case 4: // all(i)
                printf("%d\n", (a & mask[imask]) == mask[imask] ? 1 : 0);
                break;
            case 5: // any(i)
                printf("%d\n", (a & mask[imask]) >= 1 ? 1 : 0);
                break;
            case 6: // none(i)
                printf("%d\n", (a & mask[imask]) == 0 ? 1 : 0);
                break;
            case 7: // count(i)
                for (j = 0, count = 0; j < 64; j++) {
                    if ((a & mask[imask]) & (0x01L << j))
                        count++;
                }
                printf("%d\n", count);
                break;
            case 8: // val(i)
                printf("%lu\n", a & mask[imask]);
                break;
            //case 9: // val by hex
            //    printf("%lx\n", a);
            //    break;
        }
    }

    return 0;
}

