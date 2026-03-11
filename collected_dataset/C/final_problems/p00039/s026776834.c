#include <stdio.h>
int main(void){
  char str[101];
  int i,l,s,a[101]={0};
  while(scanf("%s",str)!=EOF){
    s=0;
    for(i=0;str[i]!='\0';i++){
      if(str[i]=='I')
	a[i]=1;
      else if(str[i]=='V')
	a[i]=5;
      else if(str[i]=='X')
	a[i]=10;
      else if(str[i]=='L')
	a[i]=50;
      else if(str[i]=='C')
	a[i]=100;
      else if(str[i]=='D')
	a[i]=500;
      else
	a[i]=1000;
    }
    l=i;
    for(i=0;i<l;i++){
      if(a[i]<a[i+1]){
	s+=a[i+1]-a[i];
	i++;
      }
      else
	s+=a[i];
    }
    printf("%d\n",s);
  }
  return 0;
}