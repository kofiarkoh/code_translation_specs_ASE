#include <stdio.h>
#include <math.h>
int is_prime_number(int n){
    int i;
    for(i=2;i<=sqrt(n);i++){
        if(n%i==0) return 0;
    }
    return 1;
}

int main(void){
    int i,j,n,p[100]={0},count=0,temp;;
    int p2=0,p4=0,p14=0,p24=0,p75=0;
    scanf("%d",&n);
    for(i=2;i<=n;i++){
        if(is_prime_number(i)){
            p[i]+=1;
        }else{
            for(j=2;j<n;j++){
                if(is_prime_number(j)){
                    if(i%j==0){
                        temp=i;
                        while(temp%j==0){
                            temp/=j;
                            p[j]++;
                        }
                    }
                }
            }
        }
    }
    for(i=2;i<=n;i++){
        if(is_prime_number(i)){
            if(p[i]>=2){ p2++;}
            if(p[i]>=4){ p4++;}
            if(p[i]>=14){ p14++;}
            if(p[i]>=24){ p24++;}
            if(p[i]>=75){ p75++;}
        }
    }
    if(p2>=3&&p4>=2){
        count+=p4*(p4-1)*(p2-2)/2;
    }
    if(p4>=2&&p14>=1){
        count+=p14*(p4-1);
    }
    if(p24>=1&&p2>=2){
        count+=p24*(p2-1);
    }
    count+=p75;
    printf("%d\n", count);
    return 0;
}