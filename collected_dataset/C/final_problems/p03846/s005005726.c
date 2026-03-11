#include <stdio.h>
#include <string.h>
#include <math.h>

int main(){

    int i,j,k;
    int N,v;

    scanf("%d",&N);

    int num[N];

    for(i=0;i<N;i++){   /*初期化*/
        num[i] = 0;
    }

    for(i=0;i<N;i++){
        scanf("%d",&v);
        num[v] += 1;
    }
    
    for(i=0;i<N;i++){   /*矛盾判定*/
        if(num[i] > 2 || num[0] > 1){
            printf("0");
            return 0;
        }
    }

    long int ans;

    if(N % 2 == 0)ans = (int)pow(2.0,(N)/2) % (1000000000 + 7);
    else ans = (int)pow(2.0,(N-1)/2) % (1000000000 + 7);

    printf("%ld",ans);

    return 0;
}