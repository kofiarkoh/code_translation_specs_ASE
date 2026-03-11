#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int main(){
        int c1x, c1y, c1z;
        int c2x, c2y, c2z;
        int z1, z2;

        double result;

        scanf("%d %d %d", &c1x, &c1y, &c1z);
        scanf("%d %d %d", &c2x, &c2y, &c2z);

        result = sqrt((c1x - c2x) * (c1x - c2x) + (c1y - c2y) * (c1y - c2y));

        z1 = c1z + c2z;
        z2 = c1z - c2z;

        if(result > z1)
                {
                        printf("4\n");
                }else if(result == z1)
                {
                        printf("3\n");
                }else if((result > abs(z2)) && (result < z1)){
                        printf("2\n");
                }else if(result == abs(z2)){
                        printf("1\n");
                }else if(result < abs(z2)){
                        printf("0\n");
                }

                return 0;
}
