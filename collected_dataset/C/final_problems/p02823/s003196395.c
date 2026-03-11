#include <stdio.h>

int main(void){

    long A = 0;
    long B = 0;
    long N = 0;
    long tmp = 0;
    long count = 0;
    long right = 0;
    long left = 0;
    long r_tmp = 0;
    long l_tmp = 0;

    scanf("%ld %ld %ld", &N, &A, &B);

    tmp = B - A;

    if (tmp % 2 == 0){
        count = tmp / 2;

    } else {
        right = N - B;
        left = A - 1;

        if (left < right){
            l_tmp = A - 1;
            count = B - 1;
            
            if (count - l_tmp == 1){
                count;

            } else {
                if ((count - l_tmp - 2) % 2 != 0){
                    count = (count - l_tmp - 1) / 2 + l_tmp;
                    printf("A\n");

                } else {
                    count = (count - l_tmp ) / 2 + l_tmp;
                    printf("B\n");

                }
            }
        } else {
            r_tmp = N - B;
            count = N - A;
            
            if (count - r_tmp == 1){
                count;

            } else {
                if ((count - r_tmp - 2) % 2 != 0){
                    count = (count - r_tmp - 1) / 2 + r_tmp;
                    printf("c\n");

                } else {
                    count = (count - r_tmp ) / 2 + r_tmp;
                    printf("d\n");

                }
            }
        }
    }
    printf("%ld", count);

    return 0;
}