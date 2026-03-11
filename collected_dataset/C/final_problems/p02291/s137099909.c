#include <stdio.h>


int main() {
    int x, y, x0, y0, x1, y1;
    int q;
    double v0x, v0y, vx, vy, k, v0d;

    scanf("%d %d %d %d", &x0, &y0, &x1, &y1);
    scanf("%d", &q);

    v0x = x1 - x0;
    v0y = y1 - y0;
    v0d = v0x*v0x+v0y*v0y;
    while (q--) {
        scanf("%d %d", &x, &y);
        vx = x - x0;
        vy = y - y0;
        k = (v0x * vx + v0y * vy)/v0d;
        printf("%.10f %.10f\n", 2*(k*v0x-vx)+x, 2*(k*v0y-vy)+y);
    }
}