#include<stdio.h>
#include<string.h>
int main(){
  int i;
  int a[3][3];
  char s[4];
  for(;;){
    scanf("%s",s);
    if(strcmp(s,"0")==0)
      break;
    for(i=0;i<3;i++){
      if(s[i]=='b')
	a[i][0]='b';
      else if(s[i]=='w')
	a[i][0]='w';
      else if(s[i]=='+')
	a[i][0]=0;
    }
    scanf("%s",s);
    for(i=0;i<3;i++){
      if(s[i]=='b')
	a[i][1]='b';
      else if(s[i]=='w')
	a[i][1]='w';
      else if(s[i]=='+')
	a[i][1]=0;
    }
    scanf("%s",s);
    for(i=0;i<3;i++){
      if(s[i]=='b')
	a[i][2]='b';
      else if(s[i]=='w')
	a[i][2]='w';
      else if(s[i]=='+')
	a[i][2]=0;
    }
    for(i=0;i<3;i++){
      if(a[i][0]&&a[i][0]==a[i][1]&&a[i][1]==a[i][2]){
	printf("%c\n",a[i][0]);
	break;
      }
      if(a[0][i]&&a[0][i]==a[1][i]&&a[1][i]==a[2][i]){
	printf("%c\n",a[0][i]);
	break;
      }
    }
    if(i==3){
      if(a[0][0]&&a[0][0]==a[1][1]&&a[1][1]==a[2][2]){
	printf("%c\n",a[0][0]);
	continue;
      }
      if(a[2][0]&&a[2][0]==a[1][1]&&a[1][1]==a[0][2]){
	printf("%c\n",a[2][0]);
	continue;
      }
    }else
      continue;
    puts("NA");
  }
  return 0;
}