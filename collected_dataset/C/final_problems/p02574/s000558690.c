#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
typedef struct pair
  {
      long long used;
      long long flag;
   } pair;

long long gcd(long long a,long long b){
    if(a<b){
        long long tmp=a;
        a=b;
        b=tmp;
    }
    long long r=a%b;
    while(r){
        a=b;
        b=r;
        r=a%b;
    }
    return b;
}
#define NUM 1000000
pair prime[NUM+2];
int main(int argv,char* argc[]){
    long long n;
    scanf("%lld",&n);
    long long a[n];
    for(long long i=0;i<n;i++){
        scanf("%lld",&a[i]);
        
    }
    long long sq_num = (long long)sqrt(NUM);
    

     for (long long i=0;i<=NUM;i++){
        prime[i].flag=1;
        prime[i].used=0;
    }
    
    prime[0].flag=0;
    prime[1].flag=1;
   
    for(long long i=2;i<=sq_num;i++){
        if (prime[i].flag==1){
            for(long long j=i;j*i<=NUM;j++){
                prime[i*j].flag=i;
            }
        }
    }
    prime[1].used=0;
    int flag=0;
    long long g=a[0];
    for(long long i=0;i<n;i++){
        if(prime[a[i]].flag>1){
            long long cpy=a[i];
            prime[1].used=0;
            long long array[100]={0},z=0;
            while(prime[cpy].flag!=1){
                if(array[z]!=prime[cpy].flag){
                    array[z]=prime[cpy].flag;
                    z++;
                }
                cpy/=prime[cpy].flag;
            }
            if(array[z-1]!=cpy){
                array[z]=cpy;
                z++;
            }
            for(int j=0;j<z;j++){
                if(prime[array[j]].used){
                    flag=1;
                }
                prime[array[j]].used++;
            }
        }else{
            if(prime[a[i]].used>0){
                flag=1;
                //printf("flag on@%lld\n",i);
            }
            prime[a[i]].used++;
        }
        g=gcd(g,a[i]);
        //printf("%d\n",g);
    }
    if(g!=1){
        printf("not coprime");
        return 0;
    }
    if(flag==1&&g==1){
        printf("setwise coprime");
        return 0;
    }
    printf("pairwise coprime");
    
    return 0;
}

