#include <stdio.h>
#include <string.h>

int t, len, sum, judge, end, vis[30], mark[105];
char sb[200010], v;

void Init() {
    sum = 0;
    judge = 1;
    memset(vis, 0, sizeof(vis));
    memset(mark, 0, sizeof(mark));
    gets(sb);
    len = strlen(sb);
    end = len - 1;
}

void Judge() {
    int bo = 0;
    for (int i = 0; i < len; i ++)
	vis[sb[i] - 'a'] ++;
    for (int i = 0; i < 26; i ++) {
	if (vis[i] % 2) {
	    bo ++;
	    if (bo == 2) {
		judge = 0;
		break;
	    }
	    v = i + 'a';
	}
    }
}

void solve() {
    for (int i = 0; i < len / 2; i ++) {
	int j;
	for (j = end; j >= i + 1; j --)
	    if (sb[j] == sb[i]) {
		mark[i] = 1;
		sum += end - j;
		for (int k = j; k < end; k ++)
		    sb[k] = sb[k + 1];
		end --;
		break;
	    }
    }
    if (len % 2) {
	for (int i = 0; i < len; i ++)
	   if (sb[i] == v && mark[i] == 0) {
	       sum += len / 2 - i;
	       break;
	   } 
    }
    if (judge)
	printf("%d\n", sum);
    else
	printf("-1\n");
}
int main() {
	Init();
	Judge();
	solve();
}