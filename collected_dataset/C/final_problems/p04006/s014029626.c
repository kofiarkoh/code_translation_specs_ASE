#include <stdio.h>
#define min(X, Y) ((X) < (Y) ? (X) : (Y))
#define max(X, Y) ((X) > (Y) ? (X) : (Y))
typedef long long ll;

const ll sqrtN = 64;
const ll INF = (1ll << 60ll);

ll n, x;
ll a[4096];
ll bucket[65];

void init(void);
ll range_min(int s, int t);//[s, t)
void update(int k, ll dat);

int main()
{
    // comment out
    
    ll ans = INF;

    init();

    scanf("%lld %lld", &n, &x);
    for (ll i = 1; i <= n; i++) {
        scanf("%lld", &a[i]);

        update(i, a[i]);
        update(i + n, a[i]);
    }

    for (ll i = 0; i < n; i++) {

        ll a_sum = 0;
        for (ll j = 1; j <= n; j++) {
            ll k = j + n;

            a_sum += range_min(k - i, k + 1);
        }

        ans = min(ans, i * x + a_sum);
    }
    
    printf("%lld\n", ans);

    return 0;
}

void init(void)
{
    for (int i = 0; i <= 2005; i++) {
        a[i] = INF;
    }

    for (int i = 0; i <= 70; i++) {
        bucket[i] = INF;
    }

    return;
}

//[s, t)
ll range_min(int s, int t)
{
    ll minv = INF;
    ll K = (n + sqrtN - 1) / sqrtN;
    ll l, r;
    
    for (int k = 0; k < K; k++) {
        l = k * sqrtN;
        r = (k + 1) * sqrtN;

        if (t <= l) {
            break;
        }
        else if (r <= s) {
            continue;
        }

        if (s <= l && r <= t) {
            minv = min(minv, bucket[k]);
        }
        else {
            for (ll i = max(l, s); i < min(r, t); i++) {
                minv = min(minv, a[i]);
            }
        }
        
    }

    return minv;
}

void update(int k, ll dat)
{
    ll minv = INF;
    a[k] = dat;

    k /= sqrtN;
    for (ll i = k * sqrtN; i < (k + 1) * sqrtN; i++) {
        minv = min(minv, a[k]);
    }

    bucket[k] = minv;

    return;
}
