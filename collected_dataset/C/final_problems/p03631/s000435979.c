#include<stdio.h>
int main()
{
    int n,i,d,s,number=0;
    scanf("%d",&n);
    int k = n;
    while(n%10 != 0){
        d=n%10;
        s=n/10;
        number=(number*10)+d;
        n=s;
    }

    if(number == k){
        printf("Yes");
    }
    else{
        printf("No");
    }

    return 0;
}
