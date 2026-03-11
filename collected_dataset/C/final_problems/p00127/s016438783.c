#include<stdio.h>
#include<string.h>
int main(){
  char str[250],ans[250],tmp[10][10]={"abcde","fghij","klmno","pqrst","uvwxy","z.?! "};
  int i,flg,l,a,cnt;
  
  while(scanf("%s",str)!=EOF){
    for(i=0;i<240;i++)ans[i]=0;
    l=strlen(str);
    if(l%2==1){
    exit:
      printf("NA\n");
    }
    else {
      for(cnt=0,i=0;i<l;i+=2,cnt++){
	if(!(str[i]>='1'&&str[i]<='6'&&str[i+1]>='1'&&str[i+1]<='6'))goto exit;
	ans[cnt]=tmp[(str[i]-'0'-1)][(str[i+1]-'0'-1)];
      }
      str[cnt]='\0';
      printf("%s\n",ans);
    }

  }
  return 0;
}