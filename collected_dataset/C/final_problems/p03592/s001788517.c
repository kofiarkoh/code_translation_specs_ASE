#include<stdio.h>

int main(void){
    //long n=0 , m=0, k=0;
    int n=0 , m=0, k=0;

    int ans = 0;
    //scanf("%ld %ld %ld", &n, &m, &k);
    scanf("%d %d %d", &n, &m, &k);

    for(int a = 0; a <= n && ans == 0; a++){
        for(int b = 0; b <= m && ans == 0; b++){
            if( (a*m - 2*a*b + b*n) == k ){
                ans = 1;
                break;
            }
        }
    }
    if(ans) printf("Yes\n");
    else printf("No\n");
}
