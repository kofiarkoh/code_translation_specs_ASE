#include <stdio.h>
#define N 10*10*10*10*10

int gcd(int a, int b);

int main(void){

    int n, num[N], skip, numskip[N-1], tmpgcd, maxgcd=1;
    scanf("%d", &n);
    for(int i=0; i<n; i++){
        scanf("%d", &num[i]);
    }


    tmpgcd = num[1];

    for(skip=0; skip<n; skip++){
        for(int i=0; i<n-1; i++){
            if(i < skip){
                numskip[i] = num[i];
            } else {
                numskip[i] = num[i+1];
            }
        }
        for(int i=0; i<n-1; i++){
            tmpgcd = gcd(tmpgcd, numskip[i]);
        }
        if(tmpgcd>maxgcd){
            maxgcd = tmpgcd;
        }
        tmpgcd = num[0];
    }

    printf("%d", maxgcd);
    

    return 0;
}

int gcd(int a, int b){

    int tmp, r;

    if(b>a){
        tmp = a;
        a = b;
        b = tmp;
    }

    while((r = a % b) != 0){
        a = b;
        b = r;
    }
    return b;

}
