import user from 'ic:canisters/neo';

user.start(window.prompt("Enter neo number to start:")).then( ack => {
  window.alert("done");
});
