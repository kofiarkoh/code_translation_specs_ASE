#include<stdio.h>
int main () {
    int a=1,b,c=0,d,e=0,f,n;
    scanf("%d",&n);
    while(1){
        b=a;
        if(a>n)break;
        if(b%3==0){
			printf(" %d",b);
		//	if(b==3&&e==0){printf(" %d",b);}else if(b==3&&e!=0){}else{printf(" %d",b);}
            //if(++a>=n)break;
        }else if(b%10==3){
            printf(" %d",b);
            //if(++a>=n)break;
        }else{
            b/=10;
            if(b==0){
            }else if(b!=0&&b%10==3){
                if(b!=3)printf(" %d",b);
                if(++a>=n)break;
            }
         
        }
    a++;
	e++;
            }
    printf("\n");
	scanf("%d",&e);
return 0;
}
 