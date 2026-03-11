#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int digit_count(long number);

int main(){
    long N;
    scanf("%ld", &N);
    int digit = digit_count(N);
    int i,cnt = 0 ;
    long k, j;
    for (j = 300; j <= N; j++){
        int a = 0, b = 0, c = 0;
        k = j;
        while(k != 0){
            if (k%10==3 || k%10==5 || k%10 ==7){
                if (k%10 == 3){
                    a = 1;
                }
                else if (k%10 == 5){
                    b = 1;
                }
                else if (k%10 == 7){
                    c = 1;
                }
                k /= 10;
            }
            else{
                 k=0;
                 a = 0;
            }
        }
        if (a == 1 && b == 1 && c == 1){
            cnt += 1;
        }
    }
    printf("%d\n", cnt);
    

    

    return 0;
}


int digit_count(long number) { //桁数計算
        int digit = 0;
        long number_tmp = number;
        while (number_tmp != 0) {
            number_tmp /= 10;
            digit++; 
        }
    return digit;
}

