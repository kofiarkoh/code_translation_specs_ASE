#include<stdio.h>


int main(){
  int n, i, j;
  char nowC;
  int beforekey, nowkey;
  char one[5] = {'.', ',', '!', '?', ' '};
  int pone;
  char str[1024];

  scanf("%d", &n);
  for(j = 0; j < n; j++) {
    scanf("%s", str);
    nowC = '\0';
    beforekey = -1;
    for(i = 0; i < 1024; i++) {
      if(str[i] == '\0') break;
      nowkey = (int)(str[i] - '0');
      if(nowkey != beforekey) {
	beforekey = nowkey;
	switch(nowkey) {
	case 0:
	  if(nowC != '\0') {
	    printf("%c", nowC);
	    nowC = '\0';
	  }
	  break;
	case 1:
	  pone = 0;
	  nowC = one[pone];
	  break;
	case 2:
	  nowC = 'a';
	  break;
	case 3:
	  nowC = 'd';
	  break;
	case 4:
	  nowC = 'g';
	  break;
	case 5:
	  nowC = 'j';
	  break;
	case 6:
	  nowC = 'm';
	  break;
	case 7:
	  nowC = 'p';
	  break;
	case 8:
	  nowC = 't';
	  break;
	case 9:
	  nowC = 'w';
	  break;
	}
      } else {
	switch(nowkey) {
	case 0:
	  break;
	case 1:
	  pone++;
	  if(pone == 5) pone = 0;
	  nowC = one[pone];
	  break;
	case 2:
	  if(nowC == 'c') nowC = 'a';
	  else nowC++;
	  break;
	case 3:
	  if(nowC == 'f') nowC = 'd';
	  else nowC++;
	  break;
	case 4:
	  if(nowC == 'i') nowC = 'g';
	  else nowC++;
	  break;
	case 5:
	  if(nowC == 'l') nowC = 'j';
	  else nowC++;
	  break;
	case 6:
	  if(nowC == 'o') nowC = 'm';
	  else nowC++;
	  break;
	case 7:
	  if(nowC == 's') nowC = 'p';
	  else nowC++;
	  break;
	case 8:
	  if(nowC == 'v') nowC = 't';
	  else nowC++;
	  break;
	case 9:
	  if(nowC == 'z') nowC = 'w';
	  else nowC++;
	  break;
	}
      }
    }
    printf("\n");
  }
  return 0;
}