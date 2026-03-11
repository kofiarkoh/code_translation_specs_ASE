#include <stdio.h>
#include <math.h>

int main(int argc, const char * argv[])
{

    double x1, y1, x2, y2, x3, y3;
    double midx1, midx2, midy1, midy2;
    double circlex, circley;
    double slant1, slant2;
    double slice1, slice2;
    double radius;
    int i;
    int input;
    
    scanf("%d", &input);
    for (i = 0; i < input; i++) {
        scanf("%lf %lf %lf %lf %lf %lf", &x1, &y1, &x2, &y2, &x3, &y3);
        midx1 = (x1 + x2) / 2;
        midy1 = (y1 + y2) / 2;
        midx2 = (x1 + x3) / 2;
        midy2 = (y1 + y3) / 2;
        slant1 = - (x2 - x1) / (y2 - y1);
        slant2 = - (x3 - x1) / (y3 - y1);
        slice1 = midy1 - slant1 * midx1;
        slice2 = midy2 - slant2 * midx2;
        circlex = (slice2 - slice1) / (slant1 - slant2);
        circley = slant1 * circlex + slice1;
        
        if (x2 == x1) {
            circley = midy1;
            circlex = (circley - slice2) / slant2;
        }
        else if (y2 == y1) {
            circlex = midx1;
            circley = slant2 * circlex + slice2;
        }
        if(x3 == x1) {
            circley = midy2;
            circlex = (circley - slice1) / slant1;
        }
        else if (y3 == y1) {
            circlex = midx2;
            circley = slant1 * circlex + slice1;
        }
        if (x2 == x1 && y3 == y1) {
            circley = midy1;
            circlex = midx2;
        }
        else if (x3 == x1 && y2 == y1){
            circley = midy2;
            circlex = midx1;
        }
        radius = sqrt((circlex - x1) * (circlex - x1) + (circley - y1) * (circley - y1));
        printf("%0.3f %0.3f %0.3f\n", circlex, circley, radius);
    }
    return 0;
}