#include <stdio.h>

char prime[10001];

void setprime(int max){
    int i,j;

    for(i=4;i <= max;i +=2){
        prime[i] = 1;
    }
    for(i = 3; i*i <= max;i += 2){
        if(prime[i] == 0){
            for(j = i*2;j<=max;j+=i){
                prime[j]=1;
            }
        }
    }
}

int main(void){
    int n,i;
    setprime(10001);
    while(scanf("%d",&n),n){
        for(i=n;i >= 7;i--){
            if(prime[i]+prime[i-2] == 0){
                printf("%d %d\n",i-2,i);
                break;
            }
        }
    }
    return 0;
}