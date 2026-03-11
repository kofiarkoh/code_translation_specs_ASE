#include<stdio.h>
int n,o,d,x,i;
int a[200001];
int main(void)
{
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
		
		if(a[i]%4==0){
			o++;
		}else if(a[i]%2==0){
			d++;
		}else{
			x++;
		}
	}
		
		if(d==1){
			x++;
			d--;
		}
		
		if(n==d){
			puts("Yes");
		}else if(o>=x&&d>1){
			puts("Yes");
		}else if(o+1>=x&&d==0){
			puts("Yes");
		}else{
			puts("No");
		}
	
	return 0;
}