#include <stdio.h>
#include <math.h>
#include <float.h>

inline double cross_product_z(double v0x, double v0y, double vx, double vy) {
    return v0x * vy - v0y * vx;
}

int main() {
    int x, y, x0, y0, x1, y1;
    int q;
    double v0x, v0y, vx, vy, cp, cosa, denom, v0v, vv;
    const char *judge;
    const char *message[] = {
        "COUNTER_CLOCKWISE",
        "CLOCKWISE",
        "ONLINE_BACK",
        "ONLINE_FRONT",
        "ON_SEGMENT"
    };

    scanf("%d %d %d %d", &x0, &y0, &x1, &y1);
    scanf("%d", &q);

    v0x = x1 - x0;
    v0y = y1 - y0;
    
    while (q--) {
        scanf("%d %d", &x, &y);
        vx = x - x0;
        vy = y - y0;
        cp = cross_product_z(v0x, v0y, vx, vy);
        if (cp > 0) {
            judge = message[0];
        }
        else if (cp < 0) {
            judge = message[1];
        }
        else {
            v0v = v0x*v0x+v0y*v0y;
            vv = vx*vx+vy*vy;
            denom = sqrt(v0v) * sqrt(vv);
            if (denom < DBL_MIN * 10) {
                judge = message[4];
            }
            else {
                cosa = (v0x*vx + v0y*vy)/denom;
                if (cosa + 1 < DBL_MIN) {
                    judge = message[2];
                }
                else {
                    if (v0v < vv) {
                        judge = message[3];
                    }
                    else {
                        judge = message[4];
                    }
                }
            }

        }
        printf("%s\n", judge);
    }
}