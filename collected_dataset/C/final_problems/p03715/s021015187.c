#include<stdio.h>
#define min(a,b) (((a)<(b))?(a):(b))
int main(int argc, char const *argv[]){
	long h,w,ans,i,a,b,c;
	scanf("%ld %ld",&h,&w);
	if(h%3==0 || w%3==0){
		printf("0\n");
	}else{
		ans=min(w*(h%3),(w%3)*h);
		for(i=1;i<w;i++){
			a=i*h;
			b=(w-i)*(h/2);
			c=(w-i)*(h/2+h%2);
			if(a<b){
				ans=min(ans,c-a);
			}else{
				if(a>c){
					ans=min(ans,a-b);
				}else{
					ans=min(ans,c-b);
				}
			}
		}
		for(i=1;i<h;i++){
			a=i*w;
			b=(h-i)*(w/2);
			c=(h-i)*(w/2+w%2);
			if(a<b){
				ans=min(ans,c-a);
			}else{
				if(a>c){
					ans=min(ans,a-b);
				}else{
					ans=min(ans,c-b);
				}
			}
		}
		printf("%ld\n",ans);
	}
	return 0;
}