#include <stdio.h>
#include <string.h>
#define N 100

void hanten(char str[],int n){
  char t;
  int i,j;
    for (j=0;j<n/2;j++){
      t=str[j];
      str[j]=str[n-j-1];
      str[n-j-1]=t;
    }
}

int main()
{
  int i,j,cnt=0;
  char str1[N],str2[N];
  while (scanf("%s",str1)!=EOF){
    strcpy(str2,str1);
    hanten(str1,strlen(str1));
    if (strcmp(str1,str2)==0)
      cnt++;
  }
  printf("%d\n",cnt);

  return 0;
}