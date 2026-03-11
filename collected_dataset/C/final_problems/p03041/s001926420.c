#include<stdio.h>
int main(){
	char str[10];
	int a,b;
	scanf("%d %d",&a,&b);
	scanf("%s",str);
	if(scanf("%d %d",&a,&b)<2&&scanf("%s",str)<2){
	if(str[b-1]>=97&&str[b-1]<=122){
      str[b-1]=str[b-1]-32;
	}
  	else{
	str[b-1]=str[b-1]+32;
	  }
	  printf("%s",str);
printf("\n");
return 0;
	}
	else{
		return -1;
	}
}