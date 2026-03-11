#include <stdio.h>
#include <math.h>
int main(void){
    double a,b,t;
    int n;
    long int sum;
    
    while(scanf("%lf %lf %d",&a,&b,&n)!=EOF){
        for(sum=t=0;n>0;n--){
            a*=10;
            t=floor(a/b);
            //printf("%lf %ld %ld\n",a/b,(long int)t,(long int)t%10);
            sum+=(long int)t%10;
        }
        printf("%ld\n",sum);
    }
    return 0;
}