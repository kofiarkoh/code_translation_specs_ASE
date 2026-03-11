#include<stdio.h>

char p[9][4];
int map[9];
int use[9];
int dic[9];
int ans;

int check(char a, char b){
  if(a == 'w' && b == 'W')return 1;
  else if(a == 'W' && b == 'w')return 1;
  else if(a == 'b' && b == 'B')return 1;
  else if(a == 'B' && b == 'b')return 1;
  else if(a == 'g' && b == 'G')return 1;
  else if(a == 'G' && b == 'g')return 1;
  else if(a == 'r' && b == 'R')return 1;
  else if(a == 'R' && b == 'r')return 1;
  else return 0;
}

char mapdic(int num, int purse){
  return p[map[num]][(4+purse-dic[num])%4];
}

void backtrack(int num){
  int i,j;
  int f1,f2;

  if(num == 9)ans++;
  else {
    for(i=0;i<9;i++){
      if(!use[i]){
	use[i] = 1;
	map[num] = i;
	for(j=0;j<4;j++){
	  dic[num] = j;
	  f2 = 1;
	  if(num >= 3){
	    f1 = check(mapdic(num,0),mapdic(num-3,2));
	    if(!f1)f2 = 0;
	  }
	  if(num % 3 != 0){
	    f1 = check(mapdic(num,3),mapdic(num-1,1));
	    if(!f1)f2 = 0;
	  }
	  if(f2)backtrack(num+1);
	}
	use[i] = 0;
      }
    }
  }	
}

int main(void){
  int i,k,l;
  int n;

  scanf("%d",&n);

  for(k=0;k<n;k++){
    for(l=0;l<9;l++){
      for(i=0;i<4;i++){
	scanf(" %c ",&p[l][i]);
      }
    }

    for(i=0;i<9;i++)use[i] = 0;
    ans = 0;

    backtrack(0);
    printf("%d\n",ans);

  }
  return 0;
}