//2019/9/28

#include <stdio.h>
#include <math.h>

int euc_alg(int a,int b) {
    int r=a%b;
    while(r!=0) {
        a=b;
        b=r;
        r=a%b;
    }
    return b;
}

int prime(int n) {  /*素数判定機*/
    int num,i;
    if (n==2) {
        return 1;
    }
    else if (n<2 || n%2==0) {
        return 0;
    }
    else {
        num=(int)sqrt(n);
        for (i=3;i<=num;i++) {
            if (n%i==0) {
                return 0;
                break;
            }
        }
        return 1;
    }
}

int main (void) {
    int num1,num2,ans=1,temp,dum,i;
    scanf("%d %d",&num1,&num2);
    if (num2>num1) {
        temp=num1;
        num1=num2;   
        num2=temp;
    }
    temp=euc_alg(num1,num2);
    if (temp==1) {
        printf("%d\n",ans);
    }
    else {
        if (num1%2==0 && num2%2==0) ans++;
        for (i=3;i<=temp;i+=2) {
            if (prime(i)==1) {
                if (num1%i==0 && num2%i==0) ans++;
            }
        }
        printf("%d\n",ans);
    }
    return 0;
}