#include <stdio.h>
int main(void){
	int a,b,c,i,t=1;
	scanf("%d %d %d",&a,&b,&c);
	if(c<=a){
		printf("%d\n",c);
	}
	else if(a==b){
		if(c%a==0){
			printf("%d\n",c);
		}
		else {
			puts("-1");
		}
	}
    else if(c>a){
        i=a+b;
        while(1){
            if(i>60*t){
                c+=60;
                t++;
            }
            if(i<=c&&c<=i+a)
                break;
            i+=a+b;
        }
        if(c==2147483609)c=-1;
	printf("%d\n",c);
    }
	return 0;
}