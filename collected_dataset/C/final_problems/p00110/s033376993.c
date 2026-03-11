#include<stdio.h>
int main(){
  char s[127];
  int a,n,i,j;
  while(scanf("%s",s)!=EOF){
    for(i=0;i<10;i++){
      a=-1;
      for(j=n=0;s[j];j++){
	if('0'<=s[j]&&s[j]<='9'){
	  n*=10;
	  n+=s[j]-'0';
	}
	if(s[j]=='X'){
	  if(n==0&&i==0&&(('0'<=s[j+1]&&s[j+1]<='9')||s[j+1]=='X'))break;
	  n*=10;
	  n+=i;
	}
	if(s[j]=='+'){
	  a=n;
	  n=0;
	}
	if(s[j]=='='){
	  a+=n;
	  n=0;
	}
      }
      if(a==n)break;
    }
    if(i==10)printf("NA\n");
    else     printf("%d\n",i);
  }
  return 0;
}