#include<stdio.h>
#include<string.h>

 struct s
 {
    char word[32];
    int freq,len;
 };

int main(){
  int i = 0;
  int Max_f,Max_l,f,l,j;
  Max_f = 0;
  Max_l = 0;
  struct s w[500];

  
  while(scanf("%s",w[i++].word) != EOF){
    w[i-1].len = strlen(w[i-1].word);
    ++(w[i-1].freq);
    if(Max_l < w[i-1].len){
      Max_l = w[i-1].len;
      l = i-1;
    }

    for(j=0;j < i-1;j++){
      if(!(strcmp( w[j].word , w[i-1].word))) ++(w[j].freq);
      if(Max_f < w[j].freq){
        Max_f = w[j].freq;
        f = j;
      }
    }
  }

  printf("%s %s",w[f].word,w[l].word);

  return 0;
}