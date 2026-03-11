#include <stdio.h>
#include <string.h>
int main(){
    int sum=0,n,num,i;
    char a[1000000];
    scanf("%s",a);
    n=strlen(a);

    if(n==1){
        num=(a[0]-48)*2;
        for(i=0;num;i++){
            sum+=num%10;
            num/=10;
        }
        printf("%d\n",sum);
    }

    else{
        for(i=0;i<n;i++){
            sum+=a[i]-48;
        }
        printf("%d\n",sum);
    }
    return 0;
}
