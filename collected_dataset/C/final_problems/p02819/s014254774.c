#include <stdio.h>
#include <math.h>


int main(void){

    int X = 0;
    int d_X = 0.0;
    int flag = 0;
    int i = 0;

    scanf("%d", &X);

    while (flag == 0){

        d_X = sqrt((double)X);

        if (X == 2){
            flag = 1;

        } else {

            if (X % 2 == 0){
                X++;

            } else {

                for (i = 3; i <= (int)d_X; i = i + 2){
                    //printf("%d\n", i);
                    //printf("%d\n", X);
                    //printf("%d\n", X % i);


                    if (X % i == 0){
                        X++;
                        break;

                    } 
                }
                if (i >= (int)d_X){
                    flag = 1;
                }   
            }
        }
    }
    printf("%d", X);

    return 0;
}