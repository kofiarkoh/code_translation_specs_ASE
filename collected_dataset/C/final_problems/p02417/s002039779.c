#include<stdio.h>
#include<string.h>

int main(void){
char s[4906];
int count[128] = {0};
int len;
scanf("%[a-z,A-Z, . ]", s);

  len = strlen(s);
  for(int i = 0; i < len;i++) {
    count[s[i]]++;
  }
  
printf("a : %d\n",count['a']+count['A']);
printf("b : %d\n",count['b']+count['B']);
printf("c : %d\n",count['c']+count['C']);
printf("d : %d\n",count['d']+count['D']);
printf("e : %d\n",count['e']+count['E']);
printf("f : %d\n",count['f']+count['F']);
printf("g : %d\n",count['g']+count['G']);
printf("h : %d\n",count['h']+count['H']);
printf("i : %d\n",count['i']+count['I']);
printf("j : %d\n",count['j']+count['J']);
printf("k : %d\n",count['k']+count['K']);
printf("l : %d\n",count['l']+count['L']);
printf("m : %d\n",count['m']+count['M']);
printf("n : %d\n",count['n']+count['N']);
printf("o : %d\n",count['o']+count['O']);
printf("p : %d\n",count['p']+count['P']);
printf("q : %d\n",count['q']+count['Q']);
printf("r : %d\n",count['r']+count['R']);
printf("s : %d\n",count['s']+count['S']);
printf("t : %d\n",count['t']+count['T']);
printf("u : %d\n",count['u']+count['U']);
printf("v : %d\n",count['v']+count['V']);
printf("w : %d\n",count['w']+count['W']);
printf("x : %d\n",count['x']+count['X']);
printf("y : %d\n",count['y']+count['Y']);
printf("z : %d\n",count['z']+count['Z']);  
return 0;
}

