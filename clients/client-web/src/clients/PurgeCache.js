// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import Client from '../Client';

export default class PurgeCache extends Client {
  constructor(options = {}) {
    super({
      serviceName: 'purge-cache',
      serviceVersion: 'v1',
      exchangePrefix: '',
      ...options,
    });
    this.ping.entry = {"args":[],"category":"Ping Server","method":"get","name":"ping","query":[],"route":"/ping","stability":"stable","type":"function"}; // eslint-disable-line
    this.lbheartbeat.entry = {"args":[],"category":"Load Balancer Heartbeat","method":"get","name":"lbheartbeat","query":[],"route":"/__lbheartbeat__","stability":"stable","type":"function"}; // eslint-disable-line
    this.version.entry = {"args":[],"category":"Taskcluster Version","method":"get","name":"version","query":[],"route":"/__version__","stability":"stable","type":"function"}; // eslint-disable-line
    this.purgeCache.entry = {"args":["workerPoolId"],"category":"Purge-Cache Service","input":true,"method":"post","name":"purgeCache","query":[],"route":"/purge-cache/<workerPoolId>","scopes":"purge-cache:<workerPoolId>:<cacheName>","stability":"stable","type":"function"}; // eslint-disable-line
    this.allPurgeRequests.entry = {"args":[],"category":"Purge-Cache Service","method":"get","name":"allPurgeRequests","output":true,"query":["continuationToken","limit"],"route":"/purge-cache/list","scopes":"purge-cache:all-purge-requests","stability":"stable","type":"function"}; // eslint-disable-line
    this.purgeRequests.entry = {"args":["workerPoolId"],"category":"Purge-Cache Service","method":"get","name":"purgeRequests","output":true,"query":["since"],"route":"/purge-cache/<workerPoolId>","scopes":"purge-cache:purge-requests::<workerPoolId>","stability":"stable","type":"function"}; // eslint-disable-line
  }
  /* eslint-disable max-len */
  // Respond without doing anything.
  // This endpoint is used to check that the service is up.
  /* eslint-enable max-len */
  ping(...args) {
    this.validate(this.ping.entry, args);

    return this.request(this.ping.entry, args);
  }
  /* eslint-disable max-len */
  // Respond without doing anything.
  // This endpoint is used to check that the service is up.
  /* eslint-enable max-len */
  lbheartbeat(...args) {
    this.validate(this.lbheartbeat.entry, args);

    return this.request(this.lbheartbeat.entry, args);
  }
  /* eslint-disable max-len */
  // Respond with the JSON version object.
  // https://github.com/mozilla-services/Dockerflow/blob/main/docs/version_object.md
  /* eslint-enable max-len */
  version(...args) {
    this.validate(this.version.entry, args);

    return this.request(this.version.entry, args);
  }
  /* eslint-disable max-len */
  // Publish a request to purge caches named `cacheName` with
  // on `workerPoolId` workers.
  // If such a request already exists, its `before` timestamp is updated to
  // the current time.
  /* eslint-enable max-len */
  purgeCache(...args) {
    this.validate(this.purgeCache.entry, args);

    return this.request(this.purgeCache.entry, args);
  }
  /* eslint-disable max-len */
  // View all active purge requests.
  // This is useful mostly for administors to view
  // the set of open purge requests. It should not
  // be used by workers. They should use the purgeRequests
  // endpoint that is specific to their workerType and
  // provisionerId.
  /* eslint-enable max-len */
  allPurgeRequests(...args) {
    this.validate(this.allPurgeRequests.entry, args);

    return this.request(this.allPurgeRequests.entry, args);
  }
  /* eslint-disable max-len */
  // List the caches for this `workerPoolId` that should to be
  // purged if they are from before the time given in the response.
  // This is intended to be used by workers to determine which caches to purge.
  /* eslint-enable max-len */
  purgeRequests(...args) {
    this.validate(this.purgeRequests.entry, args);

    return this.request(this.purgeRequests.entry, args);
  }
}
