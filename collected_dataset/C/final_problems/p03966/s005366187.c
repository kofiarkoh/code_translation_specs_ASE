#include<stdio.h>
int main(void)
{
    unsigned long long int pre_a=0,pre_b=0;
    unsigned long long int a,b;
    int n;
    unsigned long long int ratio_a,ratio_b;
    scanf("%d",&n);
    for(int i=0;i<n;i++){
        scanf("%llu %llu",&a,&b);
        if(pre_a%a!=0){
            ratio_a=pre_a/a+1;
        }
        else{
            ratio_a=pre_a/a;
        }
        if(pre_b%b!=0){
            ratio_b=pre_b/b+1;
        }
        else{
            ratio_b=pre_b/b;
        }
        if(a<pre_a || b<pre_b){
            if(ratio_a>ratio_b){
                a*=ratio_a;
                b*=ratio_a;
            }
            else{
                a*=ratio_b;
                b*=ratio_b;
            }
        }
        pre_a=a;
        pre_b=b;
    }
    printf("%llu\n",a+b);
    return 0;
}
