#include<stdio.h>
#include<string.h>
int main()
{
  char n[102];
  scanf("%s", n);
  int k;
  scanf("%d", &k);
  int m = strlen(n);
  int ans;
  int i;
  int a, b;
  if (k == 1)
    ans = 9 * (m - 1) + (int)(n[0] - '0');

  else if (k == 2)
  {
    a = -1;
    for(i=1; i<m; i++){
      if(n[i] > '0'){
        a = i;
        break;
      }
    }
	ans = 81 * (m-1) * (m-2) / 2;
	ans += 9 * (m-1) * (n[0]-'1');
	if(a > 0){
      ans += n[a]-'0';
      ans += 9 * (m-a-1);
	}
  }
  else{
    a = b = -1;
	for (i=1; i<m; i++){
      if (n[i] > '0'){
        if (a < 0)
          a = i;
        else if (b < 0)
          b = i;
      }  
    }
    ans = 729 * (m-1) * (m-2) * (m-3) / 6;
    ans += 81 * (m-1) * (m-2) / 2 * (n[0]-'1');
    if(a > 0){
      ans += (n[a]-'1') * 9 * (m-a-1);
      ans += 81 * (m-a-1) * (m-a-2) / 2;
      if(b > 0){
		ans += n[b]-'0';
		ans += 9 * (m-b-1);
      }
    }
  }
  printf("%d\n", ans);
  return 0;
}
