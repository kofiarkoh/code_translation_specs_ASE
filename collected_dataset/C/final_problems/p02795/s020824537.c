#include <stdio.h>

int main(void){

    int H = 0;
    int W = 0;
    int N = 0;

    scanf("%d", &H);
    scanf("%d", &W);
    scanf("%d", &N);

    if (H > W) {

        if (N % H == 0){

            N = N / H;
        } else {

            N = N / H + 1;
        }
    } else {

        if (N % W == 0){

            N = N / W;
        } else {

            N = N / W + 1;
        }
    }
    printf("%d\n", N);

    return 0;
}