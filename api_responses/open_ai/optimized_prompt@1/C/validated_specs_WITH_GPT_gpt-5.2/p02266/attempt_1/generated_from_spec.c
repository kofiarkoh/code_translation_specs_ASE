#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int pos;
} Down;

typedef struct {
    int left;   /* left boundary position of the region */
    int area;   /* positive area */
} Region;

int main(void) {
    int c;
    int pos = 0;

    Down *downs = NULL;
    size_t downs_sz = 0, downs_cap = 0;

    Region *regions = NULL;
    size_t regions_sz = 0, regions_cap = 0;

    long long total_area = 0;

    /* Recognized terrain symbols:
       - downward slope: '\'
       - upward slope:   '/'
       - flat:           '_'
       Any other character is ignored except for advancing position.
    */
    while ((c = getchar()) != EOF) {
        if (c == '\\') {
            if (downs_sz == downs_cap) {
                downs_cap = downs_cap ? downs_cap * 2 : 16;
                downs = (Down *)realloc(downs, downs_cap * sizeof(*downs));
            }
            downs[downs_sz++].pos = pos;
        } else if (c == '/') {
            if (downs_sz > 0) {
                int left = downs[--downs_sz].pos;
                int area = pos - left; /* positive because pos increases and left < pos */

                /* Merge with any regions entirely within [left, pos]:
                   those are exactly the regions with left boundary > left,
                   assuming regions are stored in increasing left order.
                */
                while (regions_sz > 0 && regions[regions_sz - 1].left > left) {
                    area += regions[regions_sz - 1].area;
                    regions_sz--;
                }

                if (regions_sz == regions_cap) {
                    regions_cap = regions_cap ? regions_cap * 2 : 16;
                    regions = (Region *)realloc(regions, regions_cap * sizeof(*regions));
                }
                regions[regions_sz].left = left;
                regions[regions_sz].area = area;
                regions_sz++;

                total_area += area;
            }
        } else {
            /* flat '_' or any other character: no state change besides position advance */
        }

        pos += 1;
    }

    /* Output */
    printf("%lld\n", total_area);
    printf("%zu", regions_sz);
    for (size_t i = 0; i < regions_sz; i++) {
        printf(" %d", regions[i].area);
    }
    printf("\n");

    free(downs);
    free(regions);
    return 0;
}
