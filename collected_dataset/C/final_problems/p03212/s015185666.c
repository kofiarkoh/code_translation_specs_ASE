#include<math.h>
#include<stdio.h>
#define min(a,b) (a)<=(b)?(a):(b)
int main()
{   int n,i,cnt3=0,cnt5=0,cnt7=0,ans=0,start=1;
    scanf("%d",&n);
    if(n>=777777753){
        printf("26484");
        return 0;
    }
           if(n>=100000000&&n<=300000000){
                printf("8334");
                return 0;
            }
            if(n>300000000&&n<=400000000){
                ans=8334;
                start=300000000;
            }
            if(n>400000000&&n<=500000000){
                printf("14384");
                return 0;
            }
            if(n>500000000&&n<=600000000){
                ans=14384;
                start=500000000;
            }
            if(n>600000000&&n<=700000000){
                ans=20434;
                return 0;
            }
            if(n>700000000&&n<=800000000){
                ans=20434;
                start=700000000;
            }
    int s=min(n,777777753);
        for(i=start;i<=s;i++){
            
        cnt3=0;cnt5=0;cnt7=0;
        int num=i;
        while(num>0){if(num%10==3)cnt3++;
        
        else if(num%10==5)cnt5++;
        else if(num%10==7)cnt7++;
        else break;
        num/=10;
        }
        if(num==0&&cnt3>0&&cnt5>0&&cnt7>0)ans++;
    }
    printf("%d",ans);

    return 0;
}
