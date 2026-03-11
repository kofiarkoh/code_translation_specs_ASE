#include <stdio.h>
#include <string.h>

int replacenum(const char*,const char*,const int);
int main(void){
  char s[1000],t[1000],s_length;
  int i,m=1000;
  
  scanf("%s",s);
  scanf("%s",t);
  
  s_length = strlen(s);
  
  for(i=0;i<s_length;i++){
    //printf("%d\n",replacenum(s,t,i));
    if(replacenum(s,t,i) == -1 || s_length < i+strlen(t)) break;
    if(replacenum(s,t,i) < m){
      m=replacenum(s,t,i);
    }
  }
  
  printf("%d\n",m);
  
  return 0;
}

int replacenum(const char*s, const char*t, const int n){ // Sのn文字目からTとSを比べ何文字違うかを返す
  int i,j,s_length,t_length,cnt=0;
  
  s_length=strlen(s);
  t_length=strlen(t);
  for (i=n,j=0;i<s_length && j<t_length;i++,j++){
    if(t[j] == '\0') break;
    //else if(s[i] == '\0') return -1;
    
    if(s[i] != t[j]) cnt++;
  }
  return cnt;
}
